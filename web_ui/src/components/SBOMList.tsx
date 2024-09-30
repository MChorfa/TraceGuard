import React, { useEffect } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { RootState } from '../store';
import { setSBOMs, setLoading, setError } from '../store/sbomSlice';
import { listSBOMs } from '../services/grpcClient';
import { useTranslation } from 'react-i18next';

const SBOMList: React.FC = () => {
  const dispatch = useDispatch();
  const { sboms, loading, error } = useSelector((state: RootState) => state.sbom);
  const { t } = useTranslation();

  useEffect(() => {
    const fetchSBOMs = async () => {
      dispatch(setLoading(true));
      try {
        const result = await listSBOMs(1, 10);
        dispatch(setSBOMs(result.sboms));
      } catch (err) {
        dispatch(setError(err.message));
      } finally {
        dispatch(setLoading(false));
      }
    };

    fetchSBOMs();
  }, [dispatch]);

  if (loading) return <div>{t('loading')}</div>;
  if (error) return <div>{t('error')}: {error}</div>;

  return (
    <div>
      <h2>{t('sbomList')}</h2>
      <ul>
        {sboms.map(sbom => (
          <li key={sbom.id}>{sbom.name} - {sbom.version}</li>
        ))}
      </ul>
    </div>
  );
};

export default SBOMList;